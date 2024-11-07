<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount, onDestroy } from "svelte";
  import StatsBar from "$lib/components/StatsBar.svelte";
  import ToolBar from "$lib/components/ToolBar.svelte";
  import ProcessTable from "$lib/components/ProcessTable.svelte";
  import ProcessDetailsModal from "$lib/components/ProcessDetailsModal.svelte";
  import KillProcessModal from "$lib/components/KillProcessModal.svelte";
  import { formatStatus } from "$lib/utils";
  import { themeStore } from "$lib/stores";
  import type { Process, SystemStats, Column } from "$lib/types";

  let processes: Process[] = [];
  let systemStats: SystemStats | null = null;
  let intervalId: number;
  let error: string | null = null;
  let searchTerm = "";
  let isLoading = true;
  let currentPage = 1;
  let itemsPerPage = 15;
  let pinnedProcesses: Set<number> = new Set();
  let selectedProcess: Process | null = null;
  let showInfoModal = false;
  let showConfirmModal = false;
  let processToKill: Process | null = null;
  let isKilling = false;
  let statusFilter = "all";
  let refreshRate = 1000;
  let isFrozen = false;

  let columns: Column[] = [
    { id: "name", label: "Process Name", visible: true, required: true },
    { id: "pid", label: "PID", visible: true, required: true },
    {
      id: "status",
      label: "Status",
      visible: true,
      format: formatStatus,
    },
    { id: "user", label: "User", visible: true },
    {
      id: "cpu_usage",
      label: "CPU %",
      visible: true,
      format: (v) => v.toFixed(1) + "%",
    },
    {
      id: "memory_usage",
      label: "Memory",
      visible: true,
      format: (v) => (v / (1024 * 1024)).toFixed(1) + " MB",
    },
    { id: "command", label: "Command", visible: false },
    { id: "ppid", label: "Parent PID", visible: false },
  ];

  let sortConfig = {
    field: "cpu_usage" as keyof Process,
    direction: "desc" as "asc" | "desc",
  };

  $: filteredProcesses = processes.filter((process) => {
    const matchesSearch = searchTerm
      ? process.name.toLowerCase().includes(searchTerm.toLowerCase()) ||
        process.command.toLowerCase().includes(searchTerm.toLowerCase()) ||
        process.pid.toString().includes(searchTerm)
      : true;

    const matchesStatus =
      statusFilter === "all" ? true : process.status === statusFilter;

    return matchesSearch && matchesStatus;
  });

  $: sortedProcesses = filteredProcesses.sort((a, b) => {
    const aPin = pinnedProcesses.has(a.pid);
    const bPin = pinnedProcesses.has(b.pid);
    if (aPin && !bPin) return -1;
    if (!aPin && bPin) return 1;

    const aValue = a[sortConfig.field];
    const bValue = b[sortConfig.field];
    const direction = sortConfig.direction === "asc" ? 1 : -1;

    if (typeof aValue === "string" && typeof bValue === "string") {
      return direction * aValue.localeCompare(bValue);
    }
    return direction * (Number(aValue) - Number(bValue));
  });

  $: totalPages = Math.ceil(filteredProcesses.length / itemsPerPage);
  $: paginatedProcesses = sortedProcesses.slice(
    (currentPage - 1) * itemsPerPage,
    currentPage * itemsPerPage,
  );

  $: {
    // Reset to first page when filtering or changing items per page
    if (searchTerm || itemsPerPage) {
      currentPage = 1;
    }
  }

  $: {
    if (intervalId) clearInterval(intervalId);
    if (!isFrozen) {
      intervalId = setInterval(() => {
        getProcesses();
      }, refreshRate);
    }
  }

  async function getProcesses() {
    try {
      const result = await invoke<[Process[], SystemStats]>("get_processes");
      processes = result[0];
      systemStats = result[1];
      error = null;
    } catch (e: unknown) {
      if (e instanceof Error) {
        error = e.message;
      } else {
        error = String(e);
      }
    }
  }

  async function killProcess(pid: number) {
    try {
      const success = await invoke<boolean>("kill_process", { pid });
      if (success) {
        await getProcesses();
      }
    } catch (e: unknown) {
      if (e instanceof Error) {
        error = e.message;
      } else {
        error = String(e);
      }
    }
  }

  function toggleSort(field: keyof Process) {
    if (sortConfig.field === field) {
      sortConfig.direction = sortConfig.direction === "asc" ? "desc" : "asc";
    } else {
      sortConfig.field = field;
      sortConfig.direction = "desc";
    }
  }

  function togglePin(pid: number) {
    if (pinnedProcesses.has(pid)) {
      pinnedProcesses.delete(pid);
    } else {
      pinnedProcesses.add(pid);
    }
    pinnedProcesses = pinnedProcesses; // Trigger reactivity
  }

  function showProcessDetails(process: Process) {
    selectedProcess = process;
    showInfoModal = true;
  }

  function confirmKillProcess(process: Process) {
    processToKill = process;
    showConfirmModal = true;
  }

  async function handleConfirmKill() {
    if (processToKill) {
      isKilling = true;
      try {
        await killProcess(processToKill.pid);
      } finally {
        isKilling = false;
        showConfirmModal = false;
        processToKill = null;
      }
    }
  }

  onMount(async () => {
    try {
      await Promise.all([getProcesses()]);
    } finally {
      isLoading = false;
    }

    themeStore.init();
  });

  onDestroy(() => {
    if (intervalId) clearInterval(intervalId);
  });
</script>

{#if isLoading}
  <div class="loading-container">
    <div class="loading-content">
      <div class="spinner"></div>
      <span class="loading-text">Loading processes...</span>
    </div>
  </div>
{:else}
  <main>
    {#if systemStats}
      <StatsBar {systemStats} />
    {/if}

    <ToolBar
      bind:searchTerm
      bind:statusFilter
      bind:itemsPerPage
      bind:currentPage
      bind:refreshRate
      bind:isFrozen
      {totalPages}
      totalResults={filteredProcesses.length}
      bind:columns
    />

    {#if error}
      <div class="alert">{error}</div>
    {/if}

    <ProcessTable
      processes={paginatedProcesses}
      {columns}
      {systemStats}
      {sortConfig}
      {pinnedProcesses}
      onToggleSort={toggleSort}
      onTogglePin={togglePin}
      onShowDetails={showProcessDetails}
      onKillProcess={confirmKillProcess}
    />
  </main>
{/if}

<ProcessDetailsModal
  show={showInfoModal}
  process={selectedProcess}
  onClose={() => {
    showInfoModal = false;
    selectedProcess = null;
  }}
/>

<KillProcessModal
  show={showConfirmModal}
  process={processToKill}
  {isKilling}
  onClose={() => {
    showConfirmModal = false;
    processToKill = null;
  }}
  onConfirm={handleConfirmKill}
/>

<svelte:head>
  <title>NeoHtop - Modern System Monitor</title>
  <meta
    name="description"
    content="A modern, web-based system monitoring interface inspired by htop"
  />
</svelte:head>

<style>
  :global(:root) {
    --base: #1e1e2e;
    --mantle: #181825;
    --crust: #11111b;
    --text: #cdd6f4;
    --subtext0: #a6adc8;
    --subtext1: #bac2de;
    --surface0: #313244;
    --surface1: #45475a;
    --surface2: #585b70;
    --overlay0: #6c7086;
    --overlay1: #7f849c;
    --blue: #89b4fa;
    --lavender: #b4befe;
    --sapphire: #74c7ec;
    --sky: #89dceb;
    --red: #f38ba8;
    --maroon: #eba0ac;
    --peach: #fab387;
    --yellow: #f9e2af;
    --green: #a6e3a1;
    --teal: #94e2d5;
  }

  :global(body) {
    margin: 0;
    padding: 0;
    font-family: -apple-system, BlinkMacSystemFont, sans-serif;
    background-color: var(--base);
    color: var(--text);
    -webkit-font-smoothing: antialiased;
    overflow: hidden;
    user-select: none;
  }

  main {
    height: 100vh;
    display: flex;
    flex-direction: column;
    min-width: min-content;
  }

  .alert {
    margin: 8px;
    padding: 8px 12px;
    background-color: var(--surface0);
    border: 1px solid var(--red);
    border-radius: 6px;
    color: var(--red);
    font-size: 13px;
  }

  .loading-container {
    width: 100vw;
    height: 100vh;
    display: flex;
    align-items: center;
    justify-content: center;
    background: linear-gradient(135deg, var(--base) 0%, var(--mantle) 100%);
  }

  .loading-content {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 16px;
  }

  .spinner {
    width: 40px;
    height: 40px;
    border: 3px solid var(--surface0);
    border-top-color: var(--blue);
    border-radius: 50%;
    animation: spin 1s ease-in-out infinite;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
  }

  .loading-text {
    color: var(--subtext0);
    font-size: 14px;
    font-weight: 500;
    letter-spacing: 0.5px;
    animation: pulse 2s ease-in-out infinite;
    text-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
  }

  @keyframes spin {
    to {
      transform: rotate(360deg);
    }
  }

  @keyframes pulse {
    0%,
    100% {
      opacity: 0.6;
    }
    50% {
      opacity: 1;
    }
  }
</style>
