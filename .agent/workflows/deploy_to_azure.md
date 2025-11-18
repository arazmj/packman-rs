---
description: Deploy to Azure Static Web Apps
---

# Deploy to Azure Static Web Apps

This guide explains how to deploy the Pacman Rust WASM game to Azure Static Web Apps.

## Prerequisites
- An Azure account
- A GitHub repository for this project

## Steps

1. **Push to GitHub**
   Ensure your project is pushed to a GitHub repository.

2. **Create Static Web App in Azure Portal**
   - Go to the Azure Portal.
   - Search for "Static Web Apps" and click "Create".
   - **Basics**:
     - Subscription: Select your subscription.
     - Resource Group: Create new or select existing.
     - Name: `pacman-rs-wasm` (or similar).
     - Plan Type: Free (for hobby/personal projects).
     - Deployment details: Select "GitHub".
     - Sign in with GitHub and authorize Azure.
     - Organization/Repository/Branch: Select your repo and branch (e.g., `main`).
   - **Build Details**:
     - Build Presets: Select "Custom".
     - App location: `/`
     - Api location: (leave empty)
     - Output location: `dist`
   - Click "Review + create", then "Create".

3. **Update GitHub Action (Automatically Created)**
   Azure will automatically commit a workflow file to your repository (in `.github/workflows/`).
   You need to ensure the build step runs your build script or `wasm-pack`.

   Edit the workflow file on GitHub or pull the changes locally.
   Look for the `build_and_deploy_job`. You might need to add steps to install `wasm-pack` and run the build.

   **Example Workflow Configuration:**

   ```yaml
   - name: Build And Deploy
     id: builddeploy
     uses: Azure/static-web-apps-deploy@v1
     with:
       azure_static_web_apps_api_token: ${{ secrets.AZURE_STATIC_WEB_APPS_API_TOKEN_... }}
       repo_token: ${{ secrets.GITHUB_TOKEN }}
       action: "upload"
       app_location: "/" # App source code path
       api_location: "" # Api source code path - optional
       output_location: "dist" # Built app content directory - optional
       # Custom build command
       app_build_command: "curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh && ./build.sh"
   ```

   *Note: The `app_build_command` is key here. It installs `wasm-pack` and runs your `./build.sh`.*

4. **Verify Deployment**
   - Once the GitHub Action completes, go to the Azure Portal resource.
   - Click on the "URL" to view your deployed game.

## Alternative: Azure CLI

If you have the Azure CLI (`az`) and Static Web Apps CLI (`swa`) installed:

1. **Build locally**:
   ```bash
   ./build.sh
   ```

2. **Deploy**:
   ```bash
   swa deploy dist --env production
   ```
   (Follow the login prompts if needed)
