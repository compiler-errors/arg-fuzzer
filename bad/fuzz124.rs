
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo124(_: S1, _: S2, _: S6, _: S7) {}
        
        fn test124() { foo124(S8, S2, S8, S6, S3, S4); }
    