//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 986/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk986<F: Float>(t76145: F, t14512: F, t8568: F, t2344: F, t71876: F, t14516: F, t8526: F, t2329: F, t71882: F, t76148: F, t76151: F, t76154: F) -> (F, F, F, F, F, F, F, F) {
    let t77837 = F::cast_from(0.2993560425465952141e-1_f64) * t76145;
    let t77838 = t14512 * t8568;
    let t77839 = F::cast_from(0.68186654135613354322e-2_f64) * t77838;
    let t77840 = t71876 * t2344;
    let t77841 = F::cast_from(0.10227998120342003148e-1_f64) * t77840;
    let t77842 = t14516 * t8526;
    let t77843 = F::cast_from(0.10227998120342003148e-1_f64) * t77842;
    let t77844 = t71882 * t2329;
    let t77845 = F::cast_from(0.13637330827122670864e-1_f64) * t77844;
    let t77846 = F::cast_from(0.40911992481368012596e-1_f64) * t76148;
    let t77848 = F::cast_from(0.40911992481368012595e-1_f64) * t76151;
    let t77849 = F::cast_from(0.5454932330849068346e-1_f64) * t76154;
    (t77837, t77839, t77841, t77843, t77845, t77846, t77848, t77849)
}
