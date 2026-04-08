-- Filters tasks by optional word, status, and priority with dynamic sorting.
-- Dynamic ORDER BY uses CASE WHEN because SQLite does not support
-- parameterized column names or sort directions natively.
SELECT id, information, priority, status, COUNT(*) OVER() AS total
FROM tasks
WHERE (:word IS NULL OR information LIKE '%' || :word || '%')
  AND (:status IS NULL OR status = :status)
  AND (:priority IS NULL OR priority = :priority)
ORDER BY
  CASE WHEN :sort_by = 'priority' AND :sort_order = 'asc'  THEN priority END ASC,
  CASE WHEN :sort_by = 'priority' AND :sort_order = 'desc' THEN priority END DESC,
  CASE WHEN :sort_by = 'status'   AND :sort_order = 'asc'  THEN status   END ASC,
  CASE WHEN :sort_by = 'status'   AND :sort_order = 'desc' THEN status   END DESC,
  CASE WHEN :sort_by = 'id'       AND :sort_order = 'asc'  THEN id       END ASC,
  CASE WHEN :sort_by = 'id'       AND :sort_order = 'desc' THEN id       END DESC
LIMIT :page_size OFFSET :page * :page_size;
