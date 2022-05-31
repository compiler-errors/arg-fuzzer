
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo156(_: S5, _: S7, _: S6, _: S4) {}
        
        fn test156() { foo156(S2, S6, S2, S1, S3, S7, S1); }
    