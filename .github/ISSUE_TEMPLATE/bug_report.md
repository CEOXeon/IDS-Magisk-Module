name: Bug Report
description: Submit a bug report
title: "BUG --- "
labels: ["bug"]
assignees:
  - octocat
body:
  - type: markdown
    attributes:
      value: |
        Thanks for taking the time to fill out this bug report!
  - type: input
    id: contact
    attributes:
      label: Contact Details
      description: How can we get in touch with you if we need more info?
      placeholder: ex. email@example.com
    validations:
      required: false
  - type: textarea
    id: what-happened
    attributes:
      label: What happened?
      description: Also tell us, what did you expect to happen?
      placeholder: Tell us what you see!
      value: "A bug happened!"
    validations:
      required: true
  - type: dropdown
    id: version
    attributes:
      label: Version
      description: What version of our Module are you running?
      options:
        - 1.0 (Default)
        - Self-Build
    validations:
      required: true
  - type: dropdown
    id: browsers
    attributes:
      label: What Magisk Version are you on?
      multiple: false
      options:
        - 25
        - 26
        - Other
  - type: textarea
    id: logs
    attributes:
      label: Relevant log output
      description: Please copy and paste any relevant log output. This will be automatically formatted into code, so no need for backticks.
      render: shell
  - type: checkboxes
    id: terms
    attributes:
      label: Verify
      description: I'm not a Bot
      options:
        - label: Verify
          required: true
