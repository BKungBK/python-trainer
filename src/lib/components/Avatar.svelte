<script lang="ts">
  interface Props {
    src?: string | null;
    name: string;
    alt?: string;
    fallbackClass?: string;
    loading?: "eager" | "lazy";
  }

  let {
    src = null,
    name,
    alt = "",
    fallbackClass = "",
    loading = "lazy",
  }: Props = $props();

  let failedSource = $state<string | null>(null);
  let initial = $derived(name.trim().slice(0, 2).toUpperCase() || "?");
  let canLoadSource = $derived(
    typeof src === "string" &&
      (src.startsWith("blob:") || src.startsWith("https://")) &&
      failedSource !== src,
  );

  function handleImageError() {
    failedSource = src;
  }
</script>

{#if canLoadSource}
  <img src={src ?? undefined} {alt} {loading} onerror={handleImageError} />
{:else}
  <span class={fallbackClass} aria-hidden={alt ? undefined : "true"}>{initial}</span>
{/if}
