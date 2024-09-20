-- 1587. Bank Account Summary II
-- https://leetcode.com/problems/bank-account-summary-ii/

SELECT name,
  SUM(amount) AS balance
FROM Transactions
  RIGHT JOIN Users ON Users.account = Transactions.account
GROUP BY name
HAVING SUM(amount) > 10000