export const getOwnCanisterId = () => {
  const mainAppElement = document.querySelector("[data-app]") as HTMLElement;
  return mainAppElement.dataset.canisterId ?? import.meta.env.PUBLIC_OWN_CANISTER_ID;
}
