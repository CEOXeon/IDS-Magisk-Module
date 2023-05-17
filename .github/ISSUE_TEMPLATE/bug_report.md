name: Bug Report
description: Submit a bug report
title: "BUG --- "
labels: ["bug"]
body:
  - type: dropdown
    attributes:
      label: What version are you on?
      options:
        - 1.0 (Default)
        - Self-Build
      validations:
        required: true
        
  - type: textarea
    attributes:
      label: How to reproduce
      validations:
        required: true
