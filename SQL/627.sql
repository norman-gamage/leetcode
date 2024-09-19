-- 627. Swap Salary
-- https://leetcode.com/problems/swap-salary/

UPDATE Salary
SET sex = (
    CASE
      WHEN sex = 'm' THEN 'f'
      ELSE 'm'
    END
  )