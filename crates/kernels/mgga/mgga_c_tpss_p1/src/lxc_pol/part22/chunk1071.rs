//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1071/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1071<F: Float>(t11436: F, t366: F, t2703: F, t2785: F, t345: F, t2723: F, t9080: F, t1474: F, t11707: F, t11733: F, t11743: F, t11750: F, t11753: F, t1477: F, t220: F, t2782: F, t2786: F, t2798: F, t2799: F, t368: F, t3987: F, t3997: F, t4001: F, t4004: F, t4008: F, t9077: F, t9089: F, t9094: F, t9117: F, t948: F, t983: F, t985: F) -> F {
    let t11760 = t366 * t11436;
    let t11767 = t2785 * t2703 * t345;
    let t11771 = t9080 * t2723 * t345;
    let t11774 = t1474 * t2723;
    let t11782 = t1474 * t2703;
    let t11789 = F::new(2.0) * t3987 * t948 * t983 * t985 + t11707 * t220 * t368 + F::new(6.0) * t11733 * t1477 * t9077 - F::new(6.0) * t11743 * t1477 * t9094 + t11750 * t983 * t985 + F::new(2.0) * t11753 * t983 * t985 + t11760 * t983 * t985 - t11767 * t1477 * t2798 + t11771 * t1477 * t9117 + F::new(2.0) * t11774 * t2782 * t2786 - t11774 * t2798 * t2799 + t11782 * t983 * t985 + F::new(2.0) * t1477 * t2782 * t9089 + F::new(4.0) * t2782 * t3997 * t4001 + F::new(4.0) * t2782 * t3997 * t4004 - F::new(2.0) * t2798 * t4001 * t4008 - F::new(2.0) * t2798 * t4004 * t4008;
    t11789
}
