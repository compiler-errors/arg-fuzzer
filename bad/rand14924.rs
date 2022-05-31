
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo14924(_: S1, _: S3, _: S5, _: S6, _: S7, _: S8) {}
        
        fn test14924() { foo14924(S3, S3, S1, S6, S6, S4, S2); }
    