/**
 * 1108. Defanging an IP Address
 * https://leetcode.com/problems/defanging-an-ip-address/
 */

function defangIPaddr(address: string): string {
  return address.replaceAll(".", "[.]");
}
