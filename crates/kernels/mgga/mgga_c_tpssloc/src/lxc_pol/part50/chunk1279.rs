//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1279/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1279<F: Float>(t1983: F, t55242: F, t8493: F, t33133: F, t6880: F, t1307: F, t7752: F, t22574: F, t8643: F, t33085: F, t6535: F, t22461: F, t7461: F) -> (F, F, F, F, F) {
    let t120702 = F::new(2.0) * t1983 * t8493 * t55242;
    let t120703 = t33133 * t6880;
    let t120705 = t7752 * t1307;
    let t120708 = F::new(6.0) * t22574 * t8643 * t120705;
    let t120709 = t33085 * t6535;
    let t120711 = t22461 * t7461;
    (t120702, t120703, t120708, t120709, t120711)
}
