
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo5930(_: S4, _: S5) {}
        
        fn test5930() { foo5930(S3, S4, S6, S7, S8); }
    