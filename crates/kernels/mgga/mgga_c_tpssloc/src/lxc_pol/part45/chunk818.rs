//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 818/1056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk818<F: Float>(t22819: F, t22825: F, t22858: F, t22863: F, t22867: F, t22805: F, t22809: F, t22830: F, t22834: F, t22837: F, t22840: F, t22848: F, t22850: F, t22856: F, t22860: F) -> F {
    let t24049 = F::new(0.33643963411783659044e-4) * t22819;
    let t24050 = F::new(0.10541775202358879834e-2) * t22825;
    let t24058 = F::new(119.0) / F::new(3456.0) * t22858;
    let t24060 = F::new(35.0) / F::new(216.0) * t22863;
    let t24061 = F::new(0.22608743412718618878e-1) * t22867;
    let t24062 = F::new(0.33913115119077928316e-1) * t22805 - F::new(0.24223653656484234512e-2) * t22809 - t24049 + t24050 + F::new(0.48447307312968469024e-2) * t22830 + t22834 / F::new(96.0) + t22837 / F::new(768.0) + t22840 / F::new(8.0) + F::new(0.16956557559538964158e-1) * t22848 + F::new(5.0) / F::new(192.0) * t22850 + F::new(0.13457585364713463618e-3) * t22856 + t24058 - F::new(7.0) / F::new(576.0) * t22860 + t24060 + t24061;
    t24062
}
