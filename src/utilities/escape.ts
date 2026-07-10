module.exports = (str: string) => str.replace(/[\\'"]/g, "\\$&");

export default function escape(str: string): string {
	return str.replace(/[\\'"]/g, "\\$&");
}
