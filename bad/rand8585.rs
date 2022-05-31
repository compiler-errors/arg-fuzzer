
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo8585(_: S1, _: S3, _: S4, _: S6, _: S7) {}
        
        fn test8585() { foo8585(S5, S4, S2, S1, S5, S3, S4); }
    