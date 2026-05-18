//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 1109/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk1109<F: Float>(t1597: F, t4509: F, t10237: F, t10189: F, t344: F, t4343: F, t2986: F, t134: F, t2978: F, t4338: F, t10190: F, t4514: F) -> (F, F, F, F) {
    let t13769 = t4509 * t1597;
    let t13770 = t13769 * t10237;
    let t13779 = t10189 * t344;
    let t13780 = t13779 * t4343;
    let t13782 = F::new(0.37037037037037037036e-3) * t2986 * t13780;
    let t13783 = t134 * t2978;
    let t13784 = t13783 * t344;
    let t13785 = t13784 * t4338;
    let t13787 = F::new(0.24691358024691358024e-3) * t2986 * t13785;
    let t13788 = t10190 * t4514;
    (t13770, t13782, t13787, t13788)
}
