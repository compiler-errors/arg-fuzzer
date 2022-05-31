
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo7757(_: S4, _: S5, _: S6) {}
        
        fn test7757() { foo7757(S1, S3, S4, S7, S8); }
    