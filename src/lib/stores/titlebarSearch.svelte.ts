export const titlebarQuery = $state<{ value: string }>({ value: '' });

export function setTitlebarQuery(value: string) {
	titlebarQuery.value = value;
}
