//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 1140/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk1140<F: Float>(t11211: F, t11372: F, t14702: F, t14705: F, t14711: F, t18742: F, t18747: F, t18749: F, t18752: F, t18755: F, t18757: F, t3270: F, t5999: F) -> (F, F) {
    let t18759 = F::new(0.16504875e0) * t18742 - t11372 + F::cast_from(0.26837777777777777779e0_f64) * t14702 - t14705 - t14711 + F::cast_from(0.91983333333333333333e-1_f64) * t11211 - F::cast_from(0.412621875e-1_f64) * t18747 + F::new(0.16504875e0) * t18749 + F::new(0.82524375e-1) * t18752 + F::new(0.19419375e1) * t18755 - F::new(0.258925e1) * t18757;
    let t18761 = t3270 * t5999;
    (t18759, t18761)
}
