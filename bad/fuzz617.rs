
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo617(_: S2, _: S6, _: S1, _: S5, _: S6) {}
        
        fn test617() { foo617(S5, S6, S7, S1, S2, S3); }
    