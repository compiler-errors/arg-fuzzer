
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo15584(_: S5, _: S6, _: S8) {}
        
        fn test15584() { foo15584(S3, S6, S2, S5, S3); }
    