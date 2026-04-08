SELECT id, information, priority, status, COUNT(*) OVER() AS total
FROM tasks
ORDER BY
  CASE WHEN :sort_by = 'priority' AND :sort_order = 'asc'  THEN priority END ASC,
  CASE WHEN :sort_by = 'priority' AND :sort_order = 'desc' THEN priority END DESC,
  CASE WHEN :sort_by = 'status'   AND :sort_order = 'asc'  THEN status   END ASC,
  CASE WHEN :sort_by = 'status'   AND :sort_order = 'desc' THEN status   END DESC,
  CASE WHEN :sort_by = 'id'       AND :sort_order = 'asc'  THEN id       END ASC,
  CASE WHEN :sort_by = 'id'       AND :sort_order = 'desc' THEN id       END DESC
LIMIT :page_size OFFSET :page * :page_size;
