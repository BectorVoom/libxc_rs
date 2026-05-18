//! MGGA_X_2D_JS17 vxc unpol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_2d_js17.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn mgga_x_2d_js17_vxc_unpol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    lapl: &Array<f64>,
    tau: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    vlapl: &mut Array<f64>,
    vtau: &mut Array<f64>,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let t3 = rho[ip] / 2.0 <= dens_threshold;
        let t4 = f64::sqrt(M_PI);
        let t6 = 1.0 <= zeta_threshold;
        let t7 = zeta_threshold - 1.0;
        let t9 = piecewise5::<f64>(t6, t7, t6, -t7, 0.0);
        let t10 = 1.0 + t9;
        let t12 = f64::sqrt(zeta_threshold);
        let t14 = f64::sqrt(t10);
        let t16 = piecewise3::<f64>(t10 <= zeta_threshold, t12 * zeta_threshold, t14 * t10);
        let t17 = 1.0 / t4 * t16;
        let t18 = M_SQRT2;
        let t19 = f64::sqrt(rho[ip]);
        let t20 = t18 * t19;
        let t21 = rho[ip] * rho[ip];
        let t22 = t21 * rho[ip];
        let t23 = 1.0 / t22;
        let t24 = sigma[ip] * t23;
        let t26 = sigma[ip] * sigma[ip];
        let t27 = t21 * t21;
        let t29 = 1.0 / t27 / t21;
        let t32 = 1.0 + 0.82505922498838542062e0 * t24 + 0.25211952768090192343e-2 * t26 * t29;
        let t33 = f64::powf(t32, 1.0 / 15.0);
        let t43 = 1.0 + 0.55877026877520282455e-1 * t24 + (-0.1544e0 * tau[ip] / t21 - 0.11596246802930644802e2) / M_PI / 4.0;
        let t44 = f64::powf(t32, 1.0 / 5.0);
        let t45 = 1.0 / t44;
        let t48 = 1.0 / t33 + 2.0 / 5.0 * t43 * t45;
        let t52 = piecewise3::<f64>(t3, 0.0, -2.0 / 3.0 * t17 * t20 * t48);
        let tzk0 = 2.0 * t52;
        zk[ip] += tzk0;
        let t54 = t18 / t19;
        let t59 = 1.0 / t33 / t32;
        let t60 = 1.0 / t27;
        let t61 = sigma[ip] * t60;
        let t63 = t27 * t22;
        let t64 = 1.0 / t63;
        let t67 = -0.24751776749651562619e1 * t61 - 0.15127171660854115406e-1 * t26 * t64;
        let t73 = -0.16763108063256084736e0 * t61 + 0.24573523213388639842e-1 * tau[ip] * t23;
        let t77 = 1.0 / t44 / t32;
        let t78 = t43 * t77;
        let t81 = -t59 * t67 / 15.0 + 2.0 / 5.0 * t73 * t45 - 2.0 / 25.0 * t78 * t67;
        let t86 = piecewise3::<f64>(t3, 0.0, -t17 * t54 * t48 / 3.0 - 2.0 / 3.0 * t17 * t20 * t81);
        let tvrho0 = 2.0 * rho[ip] * t86 + 2.0 * t52;
        vrho[ip] += tvrho0;
        let t90 = sigma[ip] * t29;
        let t92 = 0.82505922498838542062e0 * t23 + 0.50423905536180384686e-2 * t90;
        let t99 = -t59 * t92 / 15.0 + 0.22350810751008112982e-1 * t23 * t45 - 2.0 / 25.0 * t78 * t92;
        let t103 = piecewise3::<f64>(t3, 0.0, -2.0 / 3.0 * t17 * t20 * t99);
        let tvsigma0 = 2.0 * rho[ip] * t103;
        vsigma[ip] += tvsigma0;
        let tvlapl0 = 0.0;
        vlapl[ip] += tvlapl0;
        let t105 = t16 * t18;
        let t107 = 1.0 / t19 / rho[ip];
        let t111 = piecewise3::<f64>(t3, 0.0, 0.18485501104083811416e-2 * t105 * t107 * t45);
        let tvtau0 = 2.0 * rho[ip] * t111;
        vtau[ip] += tvtau0;
    }
}
