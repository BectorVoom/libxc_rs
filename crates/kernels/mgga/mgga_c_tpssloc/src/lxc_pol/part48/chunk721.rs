//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 721/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk721<F: Float>(t1906: F, t23012: F, t2679: F, t6657: F, t1894: F, t2710: F, t214: F, t1880: F, t1909: F, t22984: F, t22990: F, t22993: F, t23000: F, t23003: F, t23006: F, t23009: F, t2613: F, t2617: F, t6658: F, t6660: F, t808: F, t812: F) -> (F, F, F) {
    let t23013 = t23012 * t1906;
    let t23014 = F::new(0.63969658155208805863e-1) * t23013;
    let t23016 = t6657 * t2679;
    let t23020 = t1894 * t2710;
    let t23021 = t214 * t23020;
    let t23022 = t1880 * t23021;
    let t23024 = F::new(2.0) * t808 * t6660 - t812 * t22984 + F::new(0.3289868133696452873e-1) * t22990 - F::new(2.0) * t812 * t22993 + F::new(0.16449340668482264365e-1) * t23000 + t23003 - F::new(0.82246703342411321825e-2) * t23006 + F::new(2.0) * t812 * t23009 + t23014 + t2613 * t1909 - t812 * t23016 - F::new(2.0) * t2617 * t6658 + F::new(0.82246703342411321825e-2) * t23022;
    (t23013, t23022, t23024)
}
