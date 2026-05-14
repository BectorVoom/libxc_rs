//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 852/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk852<F: Float>(t78200: F, t72023: F, t8902: F, t72020: F, t8906: F, t22: F, t326: F, t8041: F, t8622: F, t5148: F, t570: F, t71949: F, t76435: F, t76440: F, t333: F, t5266: F, t77970: F) -> (F, F, F, F, F, F, F, F) {
    let t78201 = 0.27274661654245341728e-1 * t78200;
    let t78202 = t72023 * t8902;
    let t78203 = 0.20455996240684006297e-1 * t78202;
    let t78204 = t72020 * t8906;
    let t78205 = 0.27274661654245341729e-1 * t78204;
    let t78207 = t326 * t8041 * t22;
    let t78208 = t78207 * t8622;
    let t78209 = 0.20455996240684006297e-1 * t78208;
    let t78213 = t5148 * t71949 * t570;
    let t78214 = 0.79828278012425390427e-1 * t78213;
    let t78215 = 0.79828278012425390427e-1 * t76435;
    let t78216 = 0.14967802127329760705e-1 * t76440;
    let t78219 = 0.11974241701863808564e0 * t5266 * t77970 * t333;
    (t78201, t78203, t78205, t78209, t78214, t78215, t78216, t78219)
}
