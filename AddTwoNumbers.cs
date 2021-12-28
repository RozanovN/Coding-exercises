/*
  You are given two non-empty linked lists representing two non-negative integers.
  The digits are stored in reverse order, and each of their nodes contains a single digit. Add the two numbers and return the sum as a linked list.
  You may assume the two numbers do not contain any leading zero, except the number 0 itself.
  
  Example:
  Input: l1 = [2,4,3], l2 = [5,6,4]
  Output: [7,0,8]
  Explanation: 342 + 465 = 807.
*/
/**
 * Definition for singly-linked list.
 * public class ListNode {
 *     public int val;
 *     public ListNode next;
 *     public ListNode(int val=0, ListNode next=null) {
 *         this.val = val;
 *         this.next = next;
 *     }
 * }
 */
public class Solution {
    public ListNode AddTwoNumbers(ListNode l1, ListNode l2) {
        if (l1.next != null && l2.next != null) {
                l1.next.val += AddNumbers(l1.val, l2.val);
                return new ListNode((l1.val + l2.val) % 10, AddTwoNumbers(l1.next, l2.next));
        }
        if (l1.next == null && l2.next == null){
            if (l1.val + l2.val <= 9) return new ListNode(l1.val + l2.val);
            ListNode temp = new ListNode(1);
            return new ListNode((l1.val + l2.val) % 10, temp);
        }
        if (l2.next == null) {
            l1.next.val += AddNumbers(l1.val, l2.val);
            ListNode temp = new ListNode(0);
            return new ListNode((l1.val + l2.val) % 10, AddTwoNumbers(l1.next, temp));
        }
        else {
            l2.next.val += AddNumbers(l1.val, l2.val);
            ListNode temp = new ListNode(0);
            return new ListNode((l1.val + l2.val) % 10, AddTwoNumbers(temp, l2.next));
        }
    }
    
    private int AddNumbers(int num1, int num2)
    {
        return num1 + num2 > 9 ? 1 : 0;
    }
        
}
