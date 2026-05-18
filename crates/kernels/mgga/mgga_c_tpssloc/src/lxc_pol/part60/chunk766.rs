//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 766/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk766<F: Float>(t26246: F, t26268: F, t27012: F, t27019: F, t27022: F, t27027: F, t28058: F, t28061: F, t28063: F, t28065: F, t28068: F, t28070: F, t28074: F, t28078: F, t28080: F) -> F {
    let t28083 = t27012 + F::new(0.6728792682356731809e-4) * t26246 - t27019 + F::new(0.40372756094140390854e-3) * t28058 - F::new(0.20186378047070195427e-3) * t28061 - t28063 / F::new(1536.0) - t28065 / F::new(768.0) - F::new(0.20186378047070195427e-3) * t28068 + t27022 + t28070 / F::new(16.0) + F::new(0.84782787797694820792e-2) * t28074 - F::new(0.12111826828242117256e-2) * t28078 - t28080 / F::new(48.0) + t27027 + F::new(0.16956557559538964159e-1) * t26268;
    t28083
}
