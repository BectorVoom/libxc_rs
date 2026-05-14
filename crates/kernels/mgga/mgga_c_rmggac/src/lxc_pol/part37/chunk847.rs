//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 847/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk847<F: Float>(t78068: F, t14444: F, t8377: F, t27048: F, t76337: F, t76340: F, t76343: F, t69404: F, t570: F, t71916: F, t8940: F, t71983: F, t8626: F, t3839: F, t71982: F, t8632: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t78069 = 0.68186654135613354322e-2 * t78068;
    let t78070 = t14444 * t8377;
    let t78072 = 0.35922725105591425692e0 * t27048 * t78070;
    let t78073 = 0.14967802127329760705e-1 * t76337;
    let t78077 = 0.13637330827122670865e0 * t76340;
    let t78078 = 0.5454932330849068346e-1 * t76343;
    let t78079 = 0.79828278012425390427e-1 * t69404;
    let t78083 = 0.11974241701863808564e0 * t8940 * t71916 * t570;
    let t78090 = t71983 * t8626;
    let t78091 = 0.40911992481368012592e-1 * t78090;
    let t78093 = t3839 * t71982 * t8632;
    (t78069, t78070, t78072, t78073, t78077, t78078, t78079, t78083, t78091, t78093)
}
