//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1356/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1356<F: Float>(t10110: F, t1912: F, t22979: F, t23190: F, t23215: F, t23281: F, t2713: F, t2718: F, t2719: F, t2743: F, t40852: F, t40870: F, t6632: F, t6662: F, t6663: F, t81554: F, t81559: F, t81621: F, t81683: F, t82000: F, t82060: F, t82070: F, t82071: F, t82076: F, t82079: F, t82082: F, t82117: F, t82149: F, t82186: F, t82197: F, t82209: F, t82211: F, t82246: F, t82279: F, t82304: F, t855: F, t858: F, t865: F, t866: F, t9590: F, t9593: F) -> F {
    let t82307 = -F::new(18.0) * t2713 * t23215 + F::new(12.0) * t2713 * t22979 - F::new(6.0) * t9593 * t6663 - F::new(3.0) * t40870 * t1912 - F::new(3.0) * t82071 * t866 - F::new(3.0) * t82197 * t866 - t40852 * t1912 + F::new(6.0) * t9590 * t6632 - F::new(3.0) * t23281 * t2743 + F::cast_from(0.82246703342411321825e-2_f64) * t81554 + t82304 + t82279 + t82246 - F::cast_from(0.38381794893125283518e0_f64) * t82209 - F::cast_from(0.19190897446562641759e0_f64) * t82211 + t82186 + t82149 + t82117 + F::cast_from(0.24674011002723396547e-1_f64) * t82082 - F::cast_from(0.49348022005446793095e-1_f64) * t82076 + F::cast_from(0.12337005501361698274e-1_f64) * t82079 + t82070 + F::cast_from(0.49348022005446793095e-1_f64) * t81559 - t855 * t858 * (t81621 + t81683 + t82000 + t82060) - F::new(18.0) * t855 * t10110 * t6662 * t2719 + F::new(6.0) * t855 * t2718 * t23190 * t865;
    t82307
}
