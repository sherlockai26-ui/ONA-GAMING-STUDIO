# ONA Experimental Game Manifest

This is an experimental infrastructure format for local ONA game packages. It is not the final ONA distribution format.

```json
{
  "manifestVersion": 1,
  "identity": {
    "id": "studio.example.game",
    "name": "Example Game",
    "version": "1.0.0",
    "developer": "Example Studio"
  },
  "presentation": {
    "icon": "icon.png",
    "description": "Short game description.",
    "artwork": "artwork.png"
  },
  "execution": {
    "executable": "game.exe",
    "workingDirectory": ".",
    "arguments": []
  },
  "requirements": {
    "platform": "windows",
    "architecture": "x64"
  },
  "display": {
    "fullscreen": true,
    "resolution": "1920x1080",
    "targetDisplay": "auto"
  },
  "input": {
    "profile": "ona-standard-controller-v1"
  }
}
```

All file paths are relative to the game package root. The import source can be a USB folder today or an ONA Store download later; runtime must only depend on the installed local copy.
