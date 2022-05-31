
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo319(_: S5, _: S4, _: S7, _: S3, _: S2, _: S6, _: S6, _: S1) {}
        
        fn test319() { foo319(S7, S2, S8, S1, S5, S4, S5, S6); }
    