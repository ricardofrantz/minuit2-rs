# NIST hard-dataset baseline (iminuit vs minuit2-rs)

Priority genuine-gap targets: Hahn1

ROOT runner: skipped; upstream iminuit is the C++ Minuit2 comparator for this bead.

| Dataset | iminuit s1 | iminuit s2 | minuit2-rs s1 | minuit2-rs s2 |
| --- | --- | --- | --- | --- |
| Lanczos3 | FAIL (valid=True, fval=3.3808e-05, params=False, nfcn=337) | FAIL (valid=True, fval=5.1388e-05, params=False, nfcn=226) | FAIL (valid=True, fval=0.00455507, params=False, nfcn=117) | FAIL (valid=True, fval=0.00455507, params=False, nfcn=125) |
| BoxBOD | OK (valid=True, fval=1168.01, params=True, nfcn=68) | OK (valid=True, fval=1168.01, params=True, nfcn=82) | OK (valid=True, fval=1168.01, params=True, nfcn=60) | OK (valid=True, fval=1168.01, params=True, nfcn=60) |
| MGH09 | FAIL (valid=True, fval=0.000494688, params=False, nfcn=177) | FAIL (valid=True, fval=0.000721538, params=False, nfcn=158) | FAIL (valid=True, fval=0.000948805, params=False, nfcn=49) | FAIL (valid=True, fval=0.000948805, params=False, nfcn=57) |
| Hahn1 | OK (valid=True, fval=1.53244, params=True, nfcn=581) | FAIL (valid=True, fval=44997.1, params=False, nfcn=580) | FAIL (valid=False, fval=51623.2, params=False, nfcn=72) | FAIL (valid=False, fval=46949.9, params=False, nfcn=260) |

- **Lanczos3**: parity-failure. Certified residual SS=1.61172e-08; parameter tolerance=1e-02. iminuit does not reach the certified solution from Start 2; minuit2-rs does not reach it under the same strategies.
- **BoxBOD**: parity-success. Certified residual SS=1168.01; parameter tolerance=1e-02. iminuit reaches the certified solution from Start 2; minuit2-rs reaches it under the same strategies.
- **MGH09**: parity-failure. Certified residual SS=0.000307506; parameter tolerance=1e-02. iminuit does not reach the certified solution from Start 2; minuit2-rs does not reach it under the same strategies.
- **Hahn1**: genuine gap. Certified residual SS=1.53244; parameter tolerance=1e-02. iminuit reaches the certified solution from Start 2; minuit2-rs does not reach it under the same strategies.
