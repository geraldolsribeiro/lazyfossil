(function () {
  document.addEventListener('DOMContentLoaded', function () {
    document.querySelectorAll('blockquote').forEach(function (bq) {
      var text = bq.textContent.trim().toLowerCase();
      if (text.startsWith('field note:')) bq.classList.add('lf-note', 'lf-note-tip');
    });
  });
})();
