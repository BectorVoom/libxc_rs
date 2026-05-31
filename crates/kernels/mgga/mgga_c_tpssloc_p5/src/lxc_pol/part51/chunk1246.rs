//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1246/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1246<F: Float>(t33185: F, t8657: F, t1873: F, t7801: F, t3941: F, t2039: F, t7467: F, t1458: F, t23880: F, t26523: F, t31795: F, t33192: F, t33195: F, t33627: F, t33641: F, t33643: F, t33645: F, t33653: F, t577: F, t7010: F, t7956: F, t8508: F) -> (F, F, F) {
    let t33655 = F::cast_from(27.0_f64) * t33185 * t8657;
    let t33656 = t7801 * t1873;
    let t33658 = F::cast_from(27.0_f64) * t3941 * t33656;
    let t33659 = t2039 * t7467;
    let t33661 = F::cast_from(27.0_f64) * t3941 * t33659;
    let t33662 = F::cast_from(0.45e1_f64) * t33627 * t577 + F::cast_from(0.135e2_f64) * t31795 * t1458 + t33641 + t33643 + t33645 + F::cast_from(0.135e2_f64) * t26523 * t2039 + F::cast_from(27.0_f64) * t23880 * t7956 + F::cast_from(0.135e2_f64) * t7010 * t7801 + t33653 + t33655 + t33658 + t33661 + t33192 + t33195 + t8508;
    (t33656, t33659, t33662)
}
