export const getOwnCanisterId = () => {
  const mainAppElement = document.querySelector("[data-app]") as HTMLElement;
  return mainAppElement.dataset.canisterId ?? import.meta.env.VITE_ISSUER_CANISTER_ID;
}
