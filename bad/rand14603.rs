
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo14603(_: S1, _: S3, _: S6) {}
        
        fn test14603() { foo14603(S7, S1, S7, S7); }
    