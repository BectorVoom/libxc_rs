//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1112/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1112<F: Float>(t31084: F, t33159: F, t1983: F, t16524: F, t8319: F, t1458: F, t576: F, t1873: F, t7467: F, t3941: F, t5371: F, t8326: F) -> (F, F, F, F, F, F, F, F) {
    let t33160 = t31084 * t33159;
    let t33162 = F::new(3.0) * t1983 * t33160;
    let t33184 = F::new(27.0) * t16524 * t8319;
    let t33185 = t576 * t1458;
    let t33187 = F::new(27.0) * t33185 * t8319;
    let t33188 = t1873 * t7467;
    let t33190 = F::new(54.0) * t3941 * t33188;
    let t33191 = t5371 * t8326;
    (t33160, t33162, t33184, t33185, t33187, t33188, t33190, t33191)
}
