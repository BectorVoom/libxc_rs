//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 879/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk879<F: Float>(t14495: F, t551: F, t1587: F, t3204: F, t71983: F, t8626: F, t3839: F, t71982: F, t8632: F, t69437: F, t69445: F, t25820: F, t77091: F, t27048: F, t77338: F, t14434: F, t1652: F) -> (F, F, F, F, F, F, F, F, F) {
    let t78084 = t14495 * t551;
    let t78087 = t3204 * t1587;
    let t78090 = t71983 * t8626;
    let t78091 = 0.40911992481368012592e-1 * t78090;
    let t78093 = t3839 * t71982 * t8632;
    let t78094 = 0.6818665413561335432e-1 * t78093;
    let t78098 = 0.21819729323396273382e0 * t69437;
    let t78099 = 0.54549323308490683456e-1 * t69445;
    let t78100 = t25820 * t77091;
    let t78101 = 0.8980681276397856423e-1 * t78100;
    let t78103 = 0.35922725105591425692e0 * t27048 * t77338;
    let t78104 = t14434 * t1652;
    (t78084, t78087, t78091, t78094, t78098, t78099, t78101, t78103, t78104)
}
