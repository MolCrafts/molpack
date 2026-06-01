window.BENCHMARK_DATA = {
  "lastUpdate": 1780311994442,
  "repoUrl": "https://github.com/MolCrafts/molpack",
  "entries": {
    "molpack criterion": [
      {
        "commit": {
          "author": {
            "name": "Roy Kid",
            "username": "Roy-Kid",
            "email": "lijichen365@gmail.com"
          },
          "committer": {
            "name": "Roy Kid",
            "username": "Roy-Kid",
            "email": "lijichen365@gmail.com"
          },
          "id": "c1187ea289a08a6c6c3ac7dc7971af6ae4096cc8",
          "message": "ci(bench): restore tree before gh-pages switch; run only fast micro/kernel benches (e2e is on-demand)",
          "timestamp": "2026-06-01T10:48:01Z",
          "url": "https://github.com/MolCrafts/molpack/commit/c1187ea289a08a6c6c3ac7dc7971af6ae4096cc8"
        },
        "date": 1780311294530,
        "tool": "cargo",
        "benches": [
          {
            "name": "evaluate_unscaled/fn",
            "value": 2575,
            "range": "± 2887",
            "unit": "ns/iter"
          },
          {
            "name": "evaluate_unscaled/caller",
            "value": 2690,
            "range": "± 325",
            "unit": "ns/iter"
          },
          {
            "name": "objective_dispatch/via_inherent",
            "value": 489,
            "range": "± 71",
            "unit": "ns/iter"
          },
          {
            "name": "objective_dispatch/via_dyn",
            "value": 527,
            "range": "± 79",
            "unit": "ns/iter"
          },
          {
            "name": "objective_dispatch_caller/caller_inherent",
            "value": 570,
            "range": "± 75",
            "unit": "ns/iter"
          },
          {
            "name": "objective_dispatch_caller/caller_dyn",
            "value": 579,
            "range": "± 71",
            "unit": "ns/iter"
          },
          {
            "name": "pair_kernel/compute_f/n=200_atoms=600",
            "value": 88924,
            "range": "± 18635",
            "unit": "ns/iter"
          },
          {
            "name": "pair_kernel/compute_f/n=1000_atoms=3000",
            "value": 539052,
            "range": "± 45786",
            "unit": "ns/iter"
          },
          {
            "name": "pair_kernel/compute_f/n=3000_atoms=9000",
            "value": 1993055,
            "range": "± 107323",
            "unit": "ns/iter"
          },
          {
            "name": "pair_kernel/compute_fg/n=200_atoms=600",
            "value": 95241,
            "range": "± 11089",
            "unit": "ns/iter"
          },
          {
            "name": "pair_kernel/compute_fg/n=1000_atoms=3000",
            "value": 623853,
            "range": "± 28890",
            "unit": "ns/iter"
          },
          {
            "name": "pair_kernel/compute_fg/n=3000_atoms=9000",
            "value": 2218834,
            "range": "± 80261",
            "unit": "ns/iter"
          },
          {
            "name": "partial_gradient_merge/serial/ntotat=1024",
            "value": 5689,
            "range": "± 212",
            "unit": "ns/iter"
          },
          {
            "name": "partial_gradient_merge/parallel/ntotat=1024",
            "value": 14874,
            "range": "± 645",
            "unit": "ns/iter"
          },
          {
            "name": "partial_gradient_merge/serial/ntotat=4096",
            "value": 44542,
            "range": "± 4681",
            "unit": "ns/iter"
          },
          {
            "name": "partial_gradient_merge/parallel/ntotat=4096",
            "value": 56131,
            "range": "± 5820",
            "unit": "ns/iter"
          },
          {
            "name": "partial_gradient_merge/serial/ntotat=16384",
            "value": 310738,
            "range": "± 38365",
            "unit": "ns/iter"
          },
          {
            "name": "partial_gradient_merge/parallel/ntotat=16384",
            "value": 335717,
            "range": "± 42699",
            "unit": "ns/iter"
          },
          {
            "name": "partial_gradient_merge/serial/ntotat=65536",
            "value": 1630082,
            "range": "± 207274",
            "unit": "ns/iter"
          },
          {
            "name": "partial_gradient_merge/parallel/ntotat=65536",
            "value": 1612143,
            "range": "± 225156",
            "unit": "ns/iter"
          },
          {
            "name": "run_iteration/fn",
            "value": 658,
            "range": "± 61",
            "unit": "ns/iter"
          },
          {
            "name": "run_iteration/caller",
            "value": 700,
            "range": "± 49",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "42854324+Roy-Kid@users.noreply.github.com",
            "name": "Jichen Li",
            "username": "Roy-Kid"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "0a89c02fc1176e4017c79fe36adcb7eed9e066c6",
          "message": "ci(bench): fix gh-pages publish; run all e2e examples at 3 samples (#18)\n\n* ci(bench): check out molpack at workspace root so benchmark-action can push gh-pages\n\nThe action runs at $GITHUB_WORKSPACE; with molpack under path: molpack it found\nno .git there ('fatal: not a git repository') when fetching/pushing gh-pages.\nCheck molpack out at root (molrs stays in a subdir, used only for test data).\n\n* ci(bench): checkout molpack (root) before molrs (subdir) so clean doesn't wipe molrs test-data\n\n* ci(bench): add root checkout so benchmark-action can push gh-pages; keep molpack/molrs siblings for the path-dep build\n\n* ci(bench): build against published molrs crate (strip local path deps); checkout molpack at root for gh-pages push\n\n* ci(bench): put sed in a block scalar (the ' #' was parsed as a YAML comment)\n\n* ci(bench): TEMP smoke — run one fast micro-bench to prove gh-pages flow (restore full suite after)\n\n* ci(bench): restore tree before gh-pages switch; run only fast micro/kernel benches (e2e is on-demand)\n\n* bench(e2e): 3 samples per example (was 10); rely on natural convergence\n\n* ci(bench): run all e2e example cases (3 samples) + kernel benches; full gh-pages flow verified",
          "timestamp": "2026-06-01T12:59:33+02:00",
          "tree_id": "f15398f9aef72153e6374a98041f52547db0565e",
          "url": "https://github.com/MolCrafts/molpack/commit/0a89c02fc1176e4017c79fe36adcb7eed9e066c6"
        },
        "date": 1780311994117,
        "tool": "cargo",
        "benches": [
          {
            "name": "evaluate_unscaled/fn",
            "value": 2994,
            "range": "± 366",
            "unit": "ns/iter"
          },
          {
            "name": "evaluate_unscaled/caller",
            "value": 2774,
            "range": "± 313",
            "unit": "ns/iter"
          },
          {
            "name": "objective_dispatch/via_inherent",
            "value": 576,
            "range": "± 126",
            "unit": "ns/iter"
          },
          {
            "name": "objective_dispatch/via_dyn",
            "value": 558,
            "range": "± 63",
            "unit": "ns/iter"
          },
          {
            "name": "objective_dispatch_caller/caller_inherent",
            "value": 626,
            "range": "± 173",
            "unit": "ns/iter"
          },
          {
            "name": "objective_dispatch_caller/caller_dyn",
            "value": 641,
            "range": "± 122",
            "unit": "ns/iter"
          },
          {
            "name": "pair_kernel/compute_f/n=200_atoms=600",
            "value": 144252,
            "range": "± 29397",
            "unit": "ns/iter"
          },
          {
            "name": "pair_kernel/compute_f/n=1000_atoms=3000",
            "value": 680449,
            "range": "± 111360",
            "unit": "ns/iter"
          },
          {
            "name": "pair_kernel/compute_f/n=3000_atoms=9000",
            "value": 2388763,
            "range": "± 336712",
            "unit": "ns/iter"
          },
          {
            "name": "pair_kernel/compute_fg/n=200_atoms=600",
            "value": 155512,
            "range": "± 27532",
            "unit": "ns/iter"
          },
          {
            "name": "pair_kernel/compute_fg/n=1000_atoms=3000",
            "value": 854451,
            "range": "± 100854",
            "unit": "ns/iter"
          },
          {
            "name": "pair_kernel/compute_fg/n=3000_atoms=9000",
            "value": 2919387,
            "range": "± 269297",
            "unit": "ns/iter"
          },
          {
            "name": "partial_gradient_merge/serial/ntotat=1024",
            "value": 6820,
            "range": "± 315",
            "unit": "ns/iter"
          },
          {
            "name": "partial_gradient_merge/parallel/ntotat=1024",
            "value": 14295,
            "range": "± 965",
            "unit": "ns/iter"
          },
          {
            "name": "partial_gradient_merge/serial/ntotat=4096",
            "value": 48954,
            "range": "± 5995",
            "unit": "ns/iter"
          },
          {
            "name": "partial_gradient_merge/parallel/ntotat=4096",
            "value": 53293,
            "range": "± 5442",
            "unit": "ns/iter"
          },
          {
            "name": "partial_gradient_merge/serial/ntotat=16384",
            "value": 340400,
            "range": "± 41894",
            "unit": "ns/iter"
          },
          {
            "name": "partial_gradient_merge/parallel/ntotat=16384",
            "value": 319555,
            "range": "± 42467",
            "unit": "ns/iter"
          },
          {
            "name": "partial_gradient_merge/serial/ntotat=65536",
            "value": 1801309,
            "range": "± 229323",
            "unit": "ns/iter"
          },
          {
            "name": "partial_gradient_merge/parallel/ntotat=65536",
            "value": 1545566,
            "range": "± 208900",
            "unit": "ns/iter"
          },
          {
            "name": "run_iteration/fn",
            "value": 718,
            "range": "± 74",
            "unit": "ns/iter"
          },
          {
            "name": "run_iteration/caller",
            "value": 755,
            "range": "± 76",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}