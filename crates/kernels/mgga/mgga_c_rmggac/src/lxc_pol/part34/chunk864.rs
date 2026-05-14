//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 864/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk864<F: Float>(t76188: F, t76190: F, t36: F, t9565: F, t305: F, t14516: F, t8537: F, t2471: F, t838: F, t2141: F, t326: F, t9530: F, t2147: F, t69146: F, t76180: F, t76182: F, t76184: F, t77863: F, t77864: F, t77868: F) -> (F, F) {
    let t77869 = 0.2727466165424534173e-1 * t76188;
    let t77870 = 0.13637330827122670865e-1 * t76190;
    let t77871 = t9565 * t36;
    let t77872 = t305 * t77871;
    let t77873 = 0.14967802127329760705e-1 * t77872;
    let t77874 = t14516 * t8537;
    let t77875 = 0.27274661654245341728e-1 * t77874;
    let t77876 = t838 * t2471;
    let t77877 = t77876 * t2141;
    let t77878 = 0.13637330827122670864e-1 * t77877;
    let t77879 = t326 * t9530;
    let t77880 = t77879 * t2147;
    let t77881 = 0.68186654135613354322e-2 * t77880;
    let t77882 = t77863 + t77864 + 0.93188427318671584245e-2 * t76180 - 0.15531404553111930708e-1 * t76182 - 0.6212561821244772283e-2 * t76184 + t77868 - t77869 - t77870 - t77873 - t69146 - t77875 - t77878 - t77881;
    (t77871, t77882)
}
