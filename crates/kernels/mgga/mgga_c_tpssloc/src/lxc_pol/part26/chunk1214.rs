//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1214/1236 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1214<F: Float>(t24826: F, t24834: F, t1209: F, t85964: F, t3032: F, t475: F, t3507: F, t7348: F, t210: F, t24810: F, t24848: F, t1090: F, t24815: F, t11624: F, t11882: F, t11904: F, t11914: F, t11915: F, t24660: F, t24776: F, t24812: F, t24820: F, t24821: F, t24849: F, t24851: F, t24858: F, t24863: F, t27532: F, t3243: F, t3610: F, t3611: F, t3612: F, t3620: F, t52531: F, t7283: F, t7327: F, t7376: F, t85836: F, t85963: F, t86015: F, t86016: F) -> (F, F, F) {
    let t86020 = t24826 * t24834;
    let t86022 = t85964 * t1209;
    let t86023 = t3032 * t475;
    let t86032 = t7348 * t3507;
    let t86036 = t24810 * t210;
    let t86037 = t86036 * t24848;
    let t86039 = t24815 * t1090;
    let t86051 = 0.10966227112321509577e-1 * t7283 * t24776 * t24858 * t3243 - 0.82246703342411321826e-2 * t24849 * t24851 * t52531 * t7376 - 0.16449340668482264365e-1 * t24849 * t86015 * t86016 - 0.16449340668482264365e-1 * t86020 + 0.82246703342411321825e-2 * t85963 * t86022 * t11882 * t86023 - 0.24674011002723396548e-1 * t24812 * t24820 * t11624 * t24821 + 6.0 * t3610 * t86032 * t3612 - 0.16449340668482264365e-1 * t86037 * t24660 * t3611 * t86039 - 0.82246703342411321826e-2 * t24849 * t7327 * t3620 * t27532 + 6.0 * t11904 * t24863 + t11914 * t85836 * t11915;
    (t86032, t86037, t86051)
}
