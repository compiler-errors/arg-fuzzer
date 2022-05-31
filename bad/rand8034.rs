
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo8034(_: S2, _: S4, _: S7, _: S8) {}
        
        fn test8034() { foo8034(S1, S7, S5, S2, S0, S1); }
    