# ZERO CI/CD Workflows

## Triggers

### On Push to Main
- **CI Pipeline** - Lint, test, build
- **Security Audit** - Vulnerability scan
- **CodeQL** - Code analysis

### On Pull Request
- **CI Pipeline** - All checks required
- **Dependency Review** - Check dependencies

### On Release Tag (v*)
- **Release Pipeline** - Build & publish
- **Deployment** - Deploy to staging/production

### On Extension Tag (ext-*)
- **Gemini Extension Release** - Build & publish extension

### Scheduled
- **Security Audit** - Daily at 2 AM UTC
- **Maintenance** - Weekly dependency updates
- **Health Checks** - Every 30 minutes

## Required Secrets

See `.github/SECRETS_TEMPLATE.md`

## Debugging Workflows

To debug locally:
```bash
brew install act
act -l                    # List all workflows
act push                  # Simulate push event
act -j test-job          # Run specific job
