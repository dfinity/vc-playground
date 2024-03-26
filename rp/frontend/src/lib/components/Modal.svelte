<div
  class="modal block overflow-y-auto bg-surface-100-800-token w-modal h-auto min-h-80 p-4 rounded-container-token shadow-xl flex flex-col gap-4"
  role="dialog"
  aria-modal="true"
>
  <header class="modal-header text-2xl font-bold"><slot name="header" /></header>
  <div class="flex-1 flex flex-col">
    <slot />
  </div>
  <div class="modal-footer flex justify-end space-x-2">
    <slot name="footer" />
  </div>
</div>
