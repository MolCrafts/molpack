window.BENCHMARK_DATA = {
  "lastUpdate": 1780311295375,
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
      }
    ]
  }
}