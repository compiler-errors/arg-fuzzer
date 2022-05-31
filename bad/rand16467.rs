
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo16467(_: S7, _: S3) {}
        
        fn test16467() { foo16467(S5, S4, S7, S8); }
    