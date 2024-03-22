<div
  class="modal block overflow-y-auto bg-surface-100-800-token w-modal h-auto p-4 space-y-4 rounded-container-token shadow-xl"
  role="dialog"
  aria-modal="true"
>
  <header class="modal-header text-2xl font-bold"><slot name="header" /></header>
  <slot />
  <div class="modal-footer flex justify-end space-x-2">
    <slot name="footer" />
  </div>
</div>
