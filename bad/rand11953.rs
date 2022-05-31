
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo11953(_: S7, _: S4, _: S3, _: S7, _: S0) {}
        
        fn test11953() { foo11953(S0, S6, S1, S3, S0, S6); }
    