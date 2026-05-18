//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1227/1400 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1227<F: Float>(t1873: F, t27863: F, t33690: F, t7266: F, t7467: F, t1458: F, t31880: F, t33142: F, t33144: F, t33146: F, t33148: F, t33150: F, t33152: F, t33154: F, t33686: F, t8446: F) -> F {
    let t33711 = t27863 * t1873;
    let t33713 = t33690 * t1873;
    let t33715 = t7266 * t7467;
    let t33720 = F::new(2.0) * t1458 * t31880 + F::new(2.0) * t33142 + F::new(2.0) * t33144 + F::new(2.0) * t33146 + t33148 + t33150 + t33152 + t33154 + t33686 + F::new(2.0) * t33711 + F::new(2.0) * t33713 + F::new(2.0) * t33715 + t8446;
    t33720
}
