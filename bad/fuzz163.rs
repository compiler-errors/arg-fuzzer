
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo163(_: S1, _: S3, _: S4, _: S5, _: S7) {}
        
        fn test163() { foo163(S6, S1, S2, S3, S6, S6); }
    