// Enhanced code copy functionality for ck documentation
(function() {
  'use strict';

  // Wait for DOM to be ready
  document.addEventListener('DOMContentLoaded', function() {
    // Find all copy buttons
    const copyButtons = document.querySelectorAll('.btn-copy');
    
    copyButtons.forEach(function(button) {
      // Add click event listener
      button.addEventListener('click', function(e) {
        e.preventDefault();
        
        // Get the code block
        const codeBlock = button.closest('.highlight');
        const code = codeBlock.querySelector('code');
        
        if (!code) return;
        
        // Get the text content
        const text = code.textContent || code.innerText;
        
        // Copy to clipboard
        if (navigator.clipboard && window.isSecureContext) {
          // Use modern clipboard API
          navigator.clipboard.writeText(text).then(function() {
            showCopySuccess(button);
          }).catch(function(err) {
            console.error('Failed to copy: ', err);
            fallbackCopy(text, button);
          });
        } else {
          // Fallback for older browsers
          fallbackCopy(text, button);
        }
      });
    });
  });

  // Fallback copy method for older browsers
  function fallbackCopy(text, button) {
    const textArea = document.createElement('textarea');
    textArea.value = text;
    textArea.style.position = 'fixed';
    textArea.style.left = '-999999px';
    textArea.style.top = '-999999px';
    document.body.appendChild(textArea);
    textArea.focus();
    textArea.select();
    
    try {
      document.execCommand('copy');
      showCopySuccess(button);
    } catch (err) {
      console.error('Fallback copy failed: ', err);
      showCopyError(button);
    } finally {
      document.body.removeChild(textArea);
    }
  }

  // Show success feedback
  function showCopySuccess(button) {
    const originalText = button.textContent;
    const originalClass = button.className;
    
    // Update button appearance
    button.textContent = 'Copied!';
    button.className = originalClass + ' copied';
    
    // Reset after 2 seconds
    setTimeout(function() {
      button.textContent = originalText;
      button.className = originalClass;
    }, 2000);
  }

  // Show error feedback
  function showCopyError(button) {
    const originalText = button.textContent;
    const originalClass = button.className;
    
    // Update button appearance
    button.textContent = 'Failed';
    button.className = originalClass.replace('btn-copy', 'btn-copy error');
    
    // Reset after 2 seconds
    setTimeout(function() {
      button.textContent = originalText;
      button.className = originalClass;
    }, 2000);
  }

  // Add keyboard shortcut for copying (Ctrl+C when code block is focused)
  document.addEventListener('keydown', function(e) {
    if (e.ctrlKey && e.key === 'c') {
      const activeElement = document.activeElement;
      const codeBlock = activeElement.closest('.highlight');
      
      if (codeBlock) {
        const copyButton = codeBlock.querySelector('.btn-copy');
        if (copyButton) {
          e.preventDefault();
          copyButton.click();
        }
      }
    }
  });

  // Add tooltip on hover
  document.addEventListener('DOMContentLoaded', function() {
    const copyButtons = document.querySelectorAll('.btn-copy');
    
    copyButtons.forEach(function(button) {
      // Add tooltip
      button.setAttribute('title', 'Click to copy code');
      
      // Add ARIA label for accessibility
      button.setAttribute('aria-label', 'Copy code to clipboard');
    });
  });
})();
